use trust_dns_proto::rr::RData;
fn check(r: &RData) {
    let _ = r.as_dnssec();
}
