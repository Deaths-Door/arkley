impl Simplify for Expression {
    fn simplify(self) -> Self {
        self.remove_unnecessary_parentheses()
    }
}
