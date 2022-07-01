(def number 0)

{
    "inc" (fn [a]
      (let [ret (+ a number)]
         (var-set #'number (+ 1 number))
         ret
      )
    )
}