(defn inc [n] (+ n 1))
(defn dec [n] (- n 1))

(defn test-loop [num]
  (println num)
  (if (> num 0)
    (recur (dec num))
    (loop [n 0]
      (println n)
       (if (< n 10)
         (recur (inc n))
         "haha"))))

(println (test-loop 10))

(defn mk-closure [number]
   (fn [n1]
     (println number n1)
     (if (> n1 0)
        (do
          (var-set #'number (dec number))
          (recur (dec n1))
        )
        nil
     )
   )
)

(def ddd (mk-closure 100))
(ddd 5)
(ddd 5)