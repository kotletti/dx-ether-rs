pub trait EtherErrorPort {
    fn message(&self) -> String;
    fn stack(&self) -> String;
}
