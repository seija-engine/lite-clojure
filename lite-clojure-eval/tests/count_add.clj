(println
  (loop [count 100000000 add 0]
    (if (> count 0)
      (recur (dec count) (+ add count))
      add)))