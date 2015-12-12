#!/usr/bin/env ruby

# Generates Rust bindings for the X-Plane plugin SDK

# Path to the rust-bindgen executable
BINDGEN = '../rust-bindgen/target/debug/bindgen'
BINDGEN_OPTIONS = '-DAPL=1 -DXPLM200 -DXPLM210 -I./SDK/CHeaders/XPLM'
# Configure library path for bindgen
ENV['DYLD_LIBRARY_PATH'] = '/Library/Developer/CommandLineTools/usr/lib'

class String
    def to_snake_case!
        gsub!(/::/, '/')
        gsub!(/([A-Z]+)([A-Z][a-z])/,'\1_\2')
        gsub!(/([a-z\d])([A-Z])/,'\1_\2')
        tr!("-", "_")
        downcase!
    end

    def to_snake_case
        dup.tap { |s| s.to_snake_case! }
    end
end

# Process core files
core_modules = []
CORE_PATTERN = /XPLM(\w+).h/
Dir.foreach('./SDK/CHeaders/XPLM') do |filename|
    match = CORE_PATTERN.match filename
    if match
        module_name = match[1].to_snake_case
        core_modules << module_name
        # Run bindgen
        `#{BINDGEN} #{BINDGEN_OPTIONS} -match #{filename} -o ./src/#{module_name}.rs ./SDK/CHeaders/XPLM/#{filename}`
    end
end

# Process widget files
widget_modules = []
WIDGET_PATTERN = /XP(\w+).h/
WIDGET_DIR = './src/widgets'
if !Dir.exist? WIDGET_DIR
    Dir.mkdir WIDGET_DIR
end
Dir.foreach('./SDK/CHeaders/Widgets') do |filename|
    match = WIDGET_PATTERN.match filename
    if match
        module_name = match[1].to_snake_case
        widget_modules << module_name
        # Run bindgen
        `#{BINDGEN} #{BINDGEN_OPTIONS} -match #{filename} -o ./src/widgets/#{module_name}.rs ./SDK/CHeaders/Widgets/#{filename}`
    end
end
