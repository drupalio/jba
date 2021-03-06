require File.expand_path('../test/generator', __FILE__)
require File.expand_path('../test/utils', __FILE__)

class JBA < Thor
  include Thor::Actions
  include ::JBA::Generator
  include ::JBA::Utils

  def self.source_root
    File.expand_path('../src/z80/templates', __FILE__)
  end

  desc 'gen_z80', 'Generate the instructions for the z80 processor in JS'
  def gen_z80
    @instructions = StringIO.new.tap{ |io| generate_z80 io }.string
    template 'instructions.tt', 'src/z80/instructions.js', :force => true
  end

  desc 'check', 'Check the js with Google closure compiler'
  def check
    exec "closure #{js_args} --warning_level VERBOSE --js_output_file /dev/null"
  end

  desc 'minify', 'Minify all of the JS into one file'
  def minify
    exec "closure #{js_args} --js_output_file jba.min.js" \
         ' --compilation_level SIMPLE_OPTIMIZATIONS'
  end

  desc 'server', 'Run the testing server and the "guard" command'
  def server
    pids = []
    pids << fork { exec 'guard' }
    pids << fork { exec 'ruby test/server.rb' }
    pids.each{ |p| begin; Process.waitpid p; rescue Interrupt; end }
  end

  protected

  def js_args
    js_files.map{ |s| '--js src/' + s }.join(' ')
  end
end
