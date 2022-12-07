use strict;
use warnings;

open(FH, '<', 'input') or die $!;

my $input = <FH>;


foreach((4..length($input)-1)) {

    my $end = $_;
    my $start = $end - 4;

    my $chars = substr $input, $start, 4;

    my $occ = 0;

    # print $chars, "\n";

    foreach my $inner (split //, $chars) {
        foreach my $outer (split //, $chars) {
            if ($inner eq $outer) {
                $occ += 1;
            }
        }
    }

    print $occ, "\n";

    if ($occ == 4) {
        print($end);

        last;
    }

    $occ = 0;


}