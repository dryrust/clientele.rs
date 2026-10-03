namespace :version do
  desc "Bump the workspace patch version and refresh Cargo.lock"
  task :bump do
    old_version = File.read('VERSION').strip
    abort 'VERSION must contain a major.minor.patch version' unless old_version.match?(/\A\d+\.\d+\.\d+\z/)
    major, minor, patch = old_version.split('.').map(&:to_i)
    new_version = "#{major}.#{minor}.#{patch + 1}"

    manifest = File.read('Cargo.toml')
    section = /(^\[workspace\.package\][^\n]*\n)(.*?)(?=^\[|\z)/m
    package = manifest.match(section)
    version = /^version\s*=\s*"#{Regexp.escape(old_version)}"(?=\s*(?:#.*)?$)/
    abort 'Cargo.toml workspace version must match VERSION' unless package && package[2].scan(version).length == 1

    updated = manifest.sub(section) do
      package[1] + package[2].sub(version, "version = \"#{new_version}\"")
    end
    originals = %w[Cargo.toml VERSION Cargo.lock].to_h do |path|
      [path, File.exist?(path) ? File.binread(path) : nil]
    end
    completed = false
    begin
      File.write('Cargo.toml', updated)
      File.write('VERSION', "#{new_version}\n")
      # Update local workspace versions while retaining locked registry dependencies.
      sh 'cargo', 'update', '--workspace', '--offline'
      completed = true
    ensure
      # A failed write or Cargo command must not consume a release version, even
      # when Cargo partially updates or creates the lockfile before failing.
      unless completed
        originals.each do |path, contents|
          if contents.nil?
            File.delete(path) if File.exist?(path)
          else
            File.binwrite(path, contents)
          end
        end
      end
    end
  end
end

desc 'Test release version tooling in isolated workspaces'
task :test do
  ruby 'tests/version_bump_test.rb'
end
