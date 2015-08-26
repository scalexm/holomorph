# helloWorld -> hello_world
sub from_camel_case($str is rw) {
    $str ~~ s / 'GID' /Gid/;
    $str ~~ s / 'UID' /Uid/;
    $str ~~ s / 'AVA' /Ava/;
    $str ~~ s:g / (<:Lu>) /_$0/;
    $str.= lc;
}

sub get_type($type) {
    given $type {
        when 'UTF' { return 'String'; }
        when 'Int' { return 'i32'; }
        when 'UInt' { return 'u32'; }
        when 'Short' { return 'i16'; }
        when 'UShort' { return 'u16'; }
        when 'Byte' { return 'i8'; }
        when 'Double' { return 'f64'; }
        when 'Float' { return 'f32'; }
        when 'VarUhInt' { return 'VarUInt'; }
        when 'VarUhShort' { return 'VarUShort'; }
        when 'VarUhLong' { return 'VarULong'; }
        when 'Boolean' { return 'bool'; }
        default { return $type; }
    }
}

sub get_vec_type($type) {
    given $type {
        when 'Short' { return ''; }
        default { return $type; }
    }
}

# find the corresponding use directive for a dofus class
sub get_use($content, $type, $dirname) {
    $content ~~ / 'package com.ankamagames.dofus.network.' (\w+)
        '.game.context' /;
    my $protocol = $0;

    if $content ~~ / 'dofus.network.$protocol' [\w || \.]+
        ".$dirname.$type;" / {
        return '';
    }

    if $content !~~ / 'dofus.network.' (\w+) '.' ([\w || \.]+) ".$type;" / {
        return '';
    }

    my $use = "use protocol::$0";
    for split '.', $1 {
        $use ~= "::$_";
    }

    $use ~~ s / 'treasureHunt' /treasure_hunt/;

    return $use ~ "::$type;";
}

sub read_file($path, $use is rw, $output is rw) {
    my $content = slurp $path;

    $content ~~ / 'public class ' $<class> = (\w+)
        ' extends ' $<base_class> = (\w+) /;
    my $class = $<class>;
    my $base_class = $<base_class>;

    $content ~~ / 'protocolId:uint = ' $<id> = (\d+) /;
    my $id = $<id>;

    # content of serializeAs function
    $content ~~ / 'function serializeAs_' \w+
        '(param1:ICustomDataOutput) : void' \s+ '{'
        $<function> = (.*) '}' \s* 'override '*
        'public function deserialize(' /;
    my $function = $<function>;

    $output ~= "impl_type!($class, $id";

    my $next_vec_type = 'Static';
    for $function.lines {
        my $name;
        my $type = '';
        given $_ {
            # base class
            when / 'super' / {
                $name = 'base';
                $type = $base_class;
                $use = $use (|) get_use($content, $base_class,
                    $path.dirname.IO.basename);
            }

            # primitive type
            when / 'write' (\w+) '(this.' (\w+) ')' / {
                $name = $1;
                $type = get_type($0);
            }

            # BooleanByteWrapper
            when / 'setFlag(' \w+ ',' \d ',this.' (\w+) / {
                $name = $0;
                $type = 'Flag';
            }

            # vector length type (VarInt/Short)
            when / 'write' (\w+) '(this.' \w+ '.length' / {
                $next_vec_type = get_vec_type($0);
            }

            # vector of primitive type
            when / 'write' (\w+) '(this.' (\w+) '[' / {
                $name = $1;
                $type = get_type($0);
                if $type ~~ 'i8' {
                    $type = 'u8';
                }
                $type = "{$next_vec_type}Vec<$type>";
            }

            # dofus class
            when / 'this.' (\w+) '.serializeAs_' (\w+) / {
                $name = $0;
                $type = $1;
                $use = $use (|) get_use($content, $1,
                    $path.dirname.IO.basename);
            }

            # polymorphic dofus class
            when / 'this.' (\w+) '.serialize' / {
                $name = $0;
                $content ~~ / "public var $name:" (\w+) /;
                $type = "{$0}Variant";
                $use = $use (|) "use protocol::variants::{$0}Variant;";
            }

            # vector of dofus class
            when / '(this.' (\w+) '[_' \w+ '_] as ' (\w+) ').serializeAs' / {
                $name = $0;
                $type = "{$next_vec_type}Vec<$1>";
                $use = $use (|) get_use($content, $1,
                    $path.dirname.IO.basename);
            }

            # vector of polymorphic dofus class
            when / '(this.' (\w+) '[_' \w+ '_] as ' (\w+) ').s' / {
                $name = $0;
                $type = "{$next_vec_type}Vec<{$1}Variant>";
                $use = $use (|) "use protocol::variants::{$1}Variant;";
            }

            default { }
        }

        if $type !~~ '' {
            from_camel_case $name;

            # avoid using rust reserved names
            if $name (elem) set('type', 'self') {
                $name ~= '_';
            }

            $output ~= ", $name| $type";
        }
    }

    $output ~= ");\n";
}

multi sub read_dir($path, $base_path, $output_path) {
    my $use = '';
    my $output = '';
    my @mods;

    for dir $path {
        if .d {
            say .Str;
            @mods.push(.basename ~~ 'treasureHunt' ??
                'treasure_hunt' !! .basename);
            read_dir $_, $base_path, $output_path;
        }
        elsif .extension ~~ "as" {
            read_file $_, $use, $output;
        }
    }

    if $output !~~ '' {
        for $use {
            $output = "$_\n" ~ $output;
        }

        $output = "use std::io::\{Read, Write\};\n"
            ~ "use io::Result;\n"
            ~ "use protocol::*;\n"
            ~ $output;
    }

    for @mods {
        $output = "pub mod $_;\n" ~ $output;
    }

    my $new_path = $output_path ~ '/' ~ $path.IO.relative($base_path);
    $new_path ~~ s / 'treasureHunt' /treasure_hunt/;
    mkdir $new_path;

    spurt $new_path ~ '/mod.rs', $output;
}

multi read_dir($base_path, $output_path) {
    read_dir $base_path, $base_path, $output_path;
}

sub good_path($path) {
    $path.IO.parts{'dirname'} ~ '/' ~ $path.IO.parts{'basename'};
}

multi sub MAIN($input_path is rw, $output_path is rw where !.IO.e) {
    $input_path = good_path($input_path);
    $output_path = good_path($output_path);

    read_dir $input_path ~ '/messages', $output_path ~ '/messages';
    read_dir $input_path ~ '/types', $output_path ~ '/types';
}

multi MAIN($, $ where .IO.e) is hidden-from-USAGE {
    say 'error: output path already exists';
}