use strict;
use warnings;

open(FH, '<', 'input') or die $!;

my $input = <FH>;


foreach((14..length($input)-1)) {

    my $end = $_;
    my $start = $end - 14;

    my $chars = substr $input, $start, 14;

    my $occ = 0;

    foreach my $inner (split //, $chars) {
        foreach my $outer (split //, $chars) {
            if ($inner eq $outer) {
                $occ += 1;
            }
        }
    }

    print $occ, "\n";

    if ($occ == 14) {
        print($end);

        last;
    }

    $occ = 0;


}