use strict;
use warnings;

use feature qw(say);

my $total = 0;


while (my $line = <>) {
    my $len = length($line) - 1;

    my $first = substr($line, 0, $len / 2);
    my $second = substr($line, $len / 2, $len);

    foreach my $char (split('', $first)) {
        my $position = index($second, $char);

        if ($position != -1) {
            if ($char eq uc $char) {
                $total += ord($char) - 65 + 27;
            }
            else {
                $total += ord($char) - 97 + 1;
            }

            last;
        }
    }



}

say $total;