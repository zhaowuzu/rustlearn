use iptables;

fn main() {
    let ipt = iptables::new(false).unwrap();
    assert!(ipt.new_chain("nat", "NEWCHAINNAME").is_ok());
    assert!(ipt.append("nat", "NEWCHAINNAME", "-j ACCEPT").is_ok());
    assert!(ipt.exists("nat", "NEWCHAINNAME", "-j ACCEPT").unwrap());
    assert!(ipt.delete("nat", "NEWCHAINNAME", "-j ACCEPT").is_ok());
    assert!(ipt.delete_chain("nat", "NEWCHAINNAME").is_ok());
}
