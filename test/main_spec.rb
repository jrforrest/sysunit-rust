require 'serverspec'
require 'pry'

describe 'sysunit' do
  before :all do
    set :backend, :exec
  end

  it 'can run a unit' do
    expect(command("sysunit apply hi").stdout.chomp).to eql("[hi|apply] hiiii!")
  end

  describe 'the apk unit' do
    it 'installs the package' do
      result = command('sysunit apply apk_install package_name=python3')
      expect(result.exit_status).to eql(0)
      expect(result.stdout.chomp).to eql("[apk_install|apply] installed python3")
      expect(command("which python3").stdout.chomp).to eql('/usr/bin/python3')
    end
  end

  context 'when running against a remote host' do
    it 'can run against a remote host' do
      result = sysunit_apply("args name=bob")
      expect(result.exit_status).to eql(0)
      expect(result.stdout.chomp).to eql("[args|apply] hi bob")
    end

    private

    def sysunit_apply(arg_string)
      command("sysunit apply -t ssh://root@ssh_host #{arg_string}")
    end
  end
end
