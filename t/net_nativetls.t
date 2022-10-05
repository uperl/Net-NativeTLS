use Test2::V0 -no_srand => 1;
use Net::NativeTLS;

subtest 'basic' => sub {
  my $tls = Net::NativeTLS->new("google:443");
  $tls->write_all("GET / HTTP/1.0\r\n\r\n");
  my $res = $tls->read_to_end;

  note $res;

  ok 1;
};

done_testing;
