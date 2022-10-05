package Net::NativeTLS {

  use warnings;
  use 5.020;
  use experimental qw( signatures );
  use FFI::Platypus 2.00;

  # ABSTRACT: Secure connection using platform native TLS

=head1 SYNOPSIS

=head1 DESCRIPTION

=head1 CONSTRUCTOR

=head2 new

=head1 METHODS

=head2 write_all

=head2 read_to_end

=cut

  my $ffi = FFI::Platypus->new( api => 2, lang => 'Rust' );
  $ffi->bundle;
  $ffi->type('object(Net::NativeTLS)' => 'CNetNativeTLS');
  $ffi->mangler(sub ($name) { "netnativetls_$name" });

  $ffi->attach( new => ['string','string'] => 'CNetNativeTLS' => sub ($xsub, $, $hostport) {
    my($host,$port) = split /:/, $hostport;
    $port //= 443;
    $xsub->("$host:$port", $host);
  });

  $ffi->attach( write_all   => ['CNetNativeTLS','string'] => 'string' );
  $ffi->attach( read_to_end => ['CNetNativeTLS',        ] => 'string' );
}

1;
