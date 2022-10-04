```haskell
    let eClick = e $> "+";
    let eInc  = eClick <$> strToF
    let bNumber = holdBehavior 0 eInc $

    strToF "+" = (+ 1)
    strToF  _ = (- 1)
```

To 

```clojure
(def eClick ($> e "+"))
(def eInc (<$> eClick 
    #(match % 
        "+" (fn [a] (+ a 1))
        "-" (fn [a] (- a 1)) 
     )
))
(def bNumber (holdBehavior 0 eInc #(% %2)))