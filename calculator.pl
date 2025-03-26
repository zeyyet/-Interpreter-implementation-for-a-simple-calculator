use strict;
use warnings;

my %variables;

sub calculate {
    my $expression = shift;

    while ($expression =~ /\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g) {
        my $var = $1;
        if (exists $variables{$var}) {
            $expression =~ s/\b$var\b/$variables{$var}/g;
        } else {
            print "hata '$var' yok\n";
            return;
        }
    }

    # Eğer payda 0sa
    if ($expression =~ m{(\d+)/\s*0}) {
        print "hata 0a bölünemez\n";
        return;
    }
    
    # eval Perldeki ifadeyi hesaplamak için kullanılır
    my $result = eval($expression);
    
    if ($@) {
        print "hata geçersiz ifade\n";
    } else {
        return $result;
    }
}

sub assign_variable {
    my ($var, $value) = @_;
    $variables{$var} = $value;
    print "$var = $value\n";
}

while (1) {
    print "bir ifade girin: (çıkmak için 'exit') ";
    my $input = <STDIN>;
    chomp($input);

    last if $input eq 'exit';
    # eğer giriş bir değişken tanımlamasıysa
    if ($input =~ /^([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.*)$/) {
        my $var = $1;
        my $value = $2;
  
        my $result = calculate($value);
        if (defined $result) {
            assign_variable($var, $result);
        }
    }
    else {
        my $result = calculate($input);
        if (defined $result) {
            print "sonuç: $result\n";
        }
    }
}
print "programdan çıkıldı.\n";
