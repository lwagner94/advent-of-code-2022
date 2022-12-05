use strict;
use warnings;

use feature qw(say);

my $total = 0;


while (my $line = <>) {
    my @splitted = split(" ", $line);

    # 0 = Rock, 1 = Paper, 2 = Scissors
    my $opponent = ord($splitted[0]) - ord("A");

    # 0 = Rock, 1 = Paper, 2 = Scissors
    my $me = ord($splitted[1]) - ord("X");

    # 0 = loose, 1 = draw, 2 = win
    my $outcome = ($me - $opponent + 1) % 3;
    $total += $me + 1 + $outcome * 3;
}

say $total;