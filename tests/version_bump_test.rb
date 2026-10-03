require 'minitest/autorun'
require 'tmpdir'
require 'fileutils'
require 'open3'
require 'rbconfig'

class VersionBumpTest < Minitest::Test
  ROOT = File.expand_path('..', __dir__)

  def with_workspace(version = '0.4.9')
    Dir.mktmpdir('clientele-version-') do |dir|
      FileUtils.cp(File.join(ROOT, 'Rakefile'), dir)
      File.write(File.join(dir, 'VERSION'), "#{version}\n")
      File.write(File.join(dir, 'Cargo.toml'), <<~TOML)
        [workspace]
        members = ["library"]
        resolver = "3"

        [workspace.package]
        version = "0.4.9" # release version
        description = "Historical reference: 0.4.9"
      TOML
      FileUtils.mkdir_p(File.join(dir, 'library', 'src'))
      File.write(File.join(dir, 'library', 'Cargo.toml'), <<~TOML)
        [package]
        name = "version-fixture"
        version.workspace = true
        edition = "2024"
      TOML
      File.write(File.join(dir, 'library', 'src', 'lib.rs'), '')
      File.write(File.join(dir, 'CHANGES.md'), "## 0.4.9\nHistorical release.\n")
      output, status = Open3.capture2e('cargo', 'generate-lockfile', '--offline', chdir: dir)
      assert status.success?, output
      yield dir
    end
  end

  def bump(dir, *options)
    Open3.capture2e(RbConfig.ruby, '-S', 'rake', *options, 'version:bump', chdir: dir)
  end

  def test_bumps_only_release_metadata_and_cargo_lock
    with_workspace do |dir|
      history = File.binread(File.join(dir, 'CHANGES.md'))
      library = File.binread(File.join(dir, 'library', 'Cargo.toml'))
      output, status = bump(dir)
      assert status.success?, output
      assert_equal "0.4.10\n", File.read(File.join(dir, 'VERSION'))
      manifest = File.read(File.join(dir, 'Cargo.toml'))
      assert_includes manifest, 'version = "0.4.10" # release version'
      assert_includes manifest, 'description = "Historical reference: 0.4.9"'
      assert_includes File.read(File.join(dir, 'Cargo.lock')), 'version = "0.4.10"'
      assert_equal history, File.binread(File.join(dir, 'CHANGES.md'))
      assert_equal library, File.binread(File.join(dir, 'library', 'Cargo.toml'))
      output, status = Open3.capture2e('cargo', 'check', '--locked', '--offline', chdir: dir)
      assert status.success?, output
    end
  end

  def test_rejects_mismatched_or_malformed_versions_before_writing
    ['0.4.8', '0.4.9-beta', 'not-a-version'].each do |version|
      with_workspace(version) do |dir|
        originals = %w[Cargo.toml Cargo.lock VERSION CHANGES.md].to_h do |name|
          [name, File.binread(File.join(dir, name))]
        end
        output, status = bump(dir)
        refute status.success?, output
        originals.each do |name, original|
          assert_equal original, File.binread(File.join(dir, name)), name
        end
      end
    end
  end

  def test_rolls_back_cargo_failure_and_allows_a_single_bump_on_retry
    [true, false].each do |lockfile_exists|
      with_workspace do |dir|
        File.delete(File.join(dir, 'Cargo.lock')) unless lockfile_exists
        originals = %w[Cargo.toml VERSION Cargo.lock CHANGES.md].to_h do |name|
          path = File.join(dir, name)
          [name, File.exist?(path) ? File.binread(path) : nil]
        end
        # Override only the external command in this subprocess, including a
        # partial lockfile write before Cargo's simulated nonzero exit.
        File.write(File.join(dir, 'fail_cargo.rb'), <<~RUBY)
          module Rake::FileUtilsExt
            def sh(*command)
              raise 'unexpected command' unless command == %w[cargo update --workspace --offline]
              File.binwrite('Cargo.lock', "partial Cargo update\\n")
              raise 'simulated Cargo failure'
            end
          end
        RUBY
        output, status = bump(dir, '-r', './fail_cargo.rb')
        refute status.success?, output
        assert_includes output, 'simulated Cargo failure'
        originals.each do |name, original|
          path = File.join(dir, name)
          if original.nil?
            refute File.exist?(path), "#{name} should remain absent"
          else
            assert_equal original, File.binread(path), name
          end
        end

        output, status = bump(dir)
        assert status.success?, output
        assert_equal "0.4.10\n", File.read(File.join(dir, 'VERSION'))
        output, status = Open3.capture2e('cargo', 'check', '--locked', '--offline', chdir: dir)
        assert status.success?, output
      end
    end
  end
end
