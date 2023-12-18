# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("ratatui_rb") do |ext|
  ext.lib_dir = "lib/ratatui_rb"
end

task default: %i[compile spec standard]
