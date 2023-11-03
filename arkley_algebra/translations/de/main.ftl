# Intereger Quadratics 

discriminant-integerquadratic
    .haupttiel = Die Diskriminante (D) wird berechnet als D = b^2 - 4ac
    .subsitiute = Ersetzen Sie nun die Werte: \textbf{"{"} = {$a}, b = { $b} ,c = {$c} {"}"} 
        erhalten wir \textbf{"{"} D = {$b}^2 - 4 * {$a} * {$b} {"}"} , was zu \textbf{"{"} D = {$result} {"}"} kommt.

roots-integerquadratic
    .zero = Da die Diskriminante (D) kleiner als 0 ist, hat die quadratische Gleichung keine reellen Wurzeln
    .one = Da die Diskriminante (D) gleich 0 ist, hat die quadratische Gleichung eine sich wiederholende reelle Wurzel; {$root }
    .two = Da die Diskriminante (D) größer als 0 ist, hat die quadratische Gleichung zwei verschiedene reelle Wurzeln; { $r1 } und { $r2 }

product-integerquadratic = Um das Produkt der Wurzeln (α und β) einer quadratischen Gleichung zu finden,
    verwenden Sie die Vieta-Formel: \textbf {"{"} (α * β) = c/a {"}"} wobei \textbf {"{"} c = { $c }{"}"} und \textbf {"{"} a = { $a } {"}"}.

sum-integerquadratic = Um die Summe der Wurzeln (α und β) einer quadratischen Gleichung zu finden,
    verwenden Sie die Vieta-Formel: \textbf {"{"} (α + β) = -b/a {"}"} wobei \textbf {"{"} b = { $b }{"}"} und \textbf {"{"} a = { $a } {"}"}.

axis-symmetry-integerquadratic = Um die Symmetrieachse einer quadratischen Gleichung zu finden, 
    verwenden Sie die Formel \textbf {"{"} x = -b / (2a) {"}"} wobei \textbf {"{"} b = { $b }{"}"} und \textbf {"{"} a = { $a } {"}"}.


# Terms 


algebric-term
    .add_impossible = \textbf {"{"} { $term1 } + { $term2 } {"}"}
        Da die Terme nicht kombinierbar sind, da sie nicht dieselben Variablen und dieselben Variablenexponenten enthalten. Das Ergebnis ist dasselbe wie oben.
    .add_possible =  \textbf {"{"} { $term3 } {"}"} Da die Terme kombinierbar sind. Füge die Koeffizienten in die obige Tabelle ein.

    .sub_impossible = \textbf {"{"} { $term1 } - { $term2 } {"}"}
        Da die Terme nicht kombinierbar sind, da sie nicht dieselben Variablen und dieselben Variablenexponenten enthalten. Das Ergebnis ist dasselbe wie oben.
    .sub_possible =  \textbf {"{"} { $term3 } {"}"} Since terms are combinable. Subtract the coefficients into the above.

    .mul = \textbf {"{"} { $term1 } * { $term2 } = { $term3 } {"}"} 
        Multiplizieren Sie die Koeffizienten und Potenzen von Variablen (einschließlich einsamer Variablen, z. B. x * yx, wenn dies der einzige Term mit dieser Variablen ist).

    .div_no_op = \textbf {"{"} { $term } / 1 = { $term } {"}"} 
        Da es durch 1 geteilt ist, können wir die Division durch 1 einfach ignorieren.

    .div_no_vars = \textbf {"{"} { $term1 } / { $term2 } = { $term3 } {"}"} 
        Teilen Sie die beiden Zahlen 

    .div_cancel_common_variables = Streichen Sie nun die gemeinsamen Variablen \textit {"{"}, die sowohl in der oberen als auch in der unteren {"}"} erscheinen.
        Hier ist die Liste der gemeinsamen Variablen : { $common }