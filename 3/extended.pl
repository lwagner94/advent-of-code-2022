use strict;
use warnings;

use feature qw(say);

my $total = 0;

open(FH, '<', "input") or die $!;

while (my $first = <FH>) {
    my $second = <FH>;
    my $third = <FH>;

    say $first, $second, $third;

    foreach my $char (split('', $first)) {
        my $position = index($second, $char);
        my $position2 = index($third, $char);

        if ($position != -1 && $position2 != -1) {
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