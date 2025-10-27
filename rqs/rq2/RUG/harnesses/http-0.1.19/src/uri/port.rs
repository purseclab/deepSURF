use std::fmt;
use super::{ErrorKind, InvalidUri};
/// The port component of a URI.
pub struct Port<T> {
    port: u16,
    repr: T,
}
impl<T> Port<T> {
    /// Returns the port number as a `u16`.
    ///
    /// # Examples
    ///
    /// Port as `u16`.
    ///
    /// ```
    /// # use http::uri::Authority;
    /// let authority: Authority = "example.org:80".parse().unwrap();
    ///
    /// let port = authority.port_part().unwrap();
    /// assert_eq!(port.as_u16(), 80);
    /// ```
    pub fn as_u16(&self) -> u16 {
        self.port
    }
}
impl<T> Port<T>
where
    T: AsRef<str>,
{
    /// Converts a `str` to a port number.
    ///
    /// The supplied `str` must be a valid u16.
    pub(crate) fn from_str(bytes: T) -> Result<Self, InvalidUri> {
        bytes
            .as_ref()
            .parse::<u16>()
            .map(|port| Port { port, repr: bytes })
            .map_err(|_| { ErrorKind::InvalidPort.into() })
    }
    /// Returns the port number as a `str`.
    ///
    /// # Examples
    ///
    /// Port as `str`.
    ///
    /// ```
    /// # use http::uri::Authority;
    /// let authority: Authority = "example.org:80".parse().unwrap();
    ///
    /// let port = authority.port_part().unwrap();
    /// assert_eq!(port.as_str(), "80");
    /// ```
    pub fn as_str(&self) -> &str {
        self.repr.as_ref()
    }
}
impl<T> fmt::Debug for Port<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Port").field(&self.port).finish()
    }
}
impl<T> fmt::Display for Port<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.port, f)
    }
}
impl<T> From<Port<T>> for u16 {
    fn from(port: Port<T>) -> Self {
        port.as_u16()
    }
}
impl<T> AsRef<str> for Port<T>
where
    T: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<T, U> PartialEq<Port<U>> for Port<T> {
    fn eq(&self, other: &Port<U>) -> bool {
        self.port == other.port
    }
}
impl<T> PartialEq<u16> for Port<T> {
    fn eq(&self, other: &u16) -> bool {
        self.port == *other
    }
}
impl<T> PartialEq<Port<T>> for u16 {
    fn eq(&self, other: &Port<T>) -> bool {
        other.port == *self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partialeq_port() {
        let port_a = Port::from_str("8080").unwrap();
        let port_b = Port::from_str("8080").unwrap();
        assert_eq!(port_a, port_b);
    }
    #[test]
    fn partialeq_port_different_reprs() {
        let port_a = Port { repr: "8081", port: 8081 };
        let port_b = Port {
            repr: String::from("8081"),
            port: 8081,
        };
        assert_eq!(port_a, port_b);
        assert_eq!(port_b, port_a);
    }
    #[test]
    fn partialeq_u16() {
        let port = Port::from_str("8080").unwrap();
        assert_eq!(port, 8080);
        assert_eq!(8080, port);
    }
    #[test]
    fn u16_from_port() {
        let port = Port::from_str("8080").unwrap();
        assert_eq!(8080, u16::from(port));
    }
}
#[cfg(test)]
mod tests_rug_484 {
    use super::*;
    use crate::uri::Authority;
    #[test]
    fn test_as_u16() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let authority: Authority = rug_fuzz_0.parse().unwrap();
        let port = authority.port_part().unwrap();
        debug_assert_eq!(port.as_u16(), 80);
             }
});    }
}
#[cfg(test)]
mod tests_rug_485 {
    use super::*;
    use crate::uri::port::{Port, InvalidUri};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = rug_fuzz_0;
        let result = Port::<&str>::from_str(bytes);
        debug_assert!(result.is_ok());
             }
});    }
}
#[cfg(test)]
mod tests_rug_486 {
    use super::*;
    use crate::uri::Authority;
    use crate::uri::port::Port;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let authority: Authority = rug_fuzz_0.parse().unwrap();
        let port = authority.port_part().unwrap();
        let p0: Port<_> = port;
        debug_assert_eq!(p0.as_str(), "80");
             }
});    }
}
