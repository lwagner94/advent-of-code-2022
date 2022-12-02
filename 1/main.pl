use strict;
use warnings;

use feature qw(say);


my @totals;

my $current = 0;

while (my $line = <>) {

    if ($line eq "\n") {
        push(@totals, $current);
        $current = 0;
    } else {
        $current += $line;
    }
}

my @sorted = sort { $b <=> $a } @totals;


say $sorted[0];
say $sorted[0] + $sorted[1] + $sorted[2];



