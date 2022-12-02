use strict;
use warnings;

use feature qw(say);

my $total = 0;


while (my $line = <>) {
    my @splitted = split(" ", $line);

    # 0 = Rock, 1 = Paper, 2 = Scissors
    my $opponent = ord($splitted[0]) - ord("A");

    # 0 = Loose, 1 = draw, 2 = win
    my $outcome = ord($splitted[1]) - ord("X");

    my $points = 0;

    my $me = ($opponent + $outcome - 1) % 3;

    $total += $me + 1 + $outcome * 3;


}

say $total;