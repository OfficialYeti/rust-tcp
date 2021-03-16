RFCs for TCP:
----------

RFC 1180
A TCP/IP Tutorial.
 - an introduction that could help to understand basics.
https://tools.ietf.org/html/rfc1180

RFC7414
A Roadmap for Transmission Control Protocol (TCP) Specification Documents
 - an guide for implementing own TCP.
https://tools.ietf.org/html/rfc7414

RFC2398
Some Testing Tools for TCP Implementors
 - an set of advices about testing
https://tools.ietf.org/html/rfc2398

RFC2525
Known TCP Implementation Problems
 - an help with debugging TCP
https://tools.ietf.org/html/rfc2525


Pnet - rust crate for low level networking, good for creating own protocol
----------

Tun-tap - The TUN/TAP allows implementing a virtual network adapter in userspace. This provides the bindings for Rust.
----------
Sucks that this is for linux only. Implementation for windows exists but I would not trust it.


Lets start::

sudo setcap cap_net_admin=eip ./trust/target/release/trust

sudo ip addr add 192.168.0.1/24 dev tun0

ip addr

sudo ip link set up dev tun0
-> thanks to this tcp recived packet

automate it in script -> run.sh
        -- chmod +x run.sh to give premission (execute i suppose)

pgrep -af target
kill

trap "kill $pid" INT TERM

https://en.wikipedia.org/wiki/EtherType
-> 0x86dd is IPv6
-> 0x800 is IPv4

IP Protocol numbers:
https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
1 is ICMP -> after ping -I tun0 192.168.0.2
6 is TCP -> after nc 192.168.0.2 80

sniff packet with 'tshark -i tun0'