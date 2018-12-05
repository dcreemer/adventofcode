;; Solution to AoC 2018 Day 05

(ns aoc.five
  (:require [clojure.string :as string]
            [clojure.test :refer [deftest is run-tests]]))

(def input
  (->> (slurp "inputs/day05.txt")))

;; part one

(defn pair? [a b]
  (and (not= a b)
       (= (string/upper-case a) (string/upper-case b))))

(defn react1 [s]
  (->> (partition 2 s)
       (reduce (fn [xs [a b]]
                 (if (pair? a b) xs (conj xs [a b])))
               [])
       (flatten)
       (string/join)))

(defn react2 [s]
  (let [s1 (react1 s)
        s2 (react1 (str " " s1 " "))]
    (string/trim s2)))

(defn react [s]
  (loop [s'  s
         s1 (react2 s')]
    (if (= s' s1)
      s'
      (recur s1 (react2 s1)))))

(deftest test-reaction
  (is (= (react "aA") ""))
  (is (= (react "abBA") ""))
  (is (= (react "abAB") "abAB"))
  (is (= (react "aabAAB") "aabAAB"))
  (is (= (react "dabAcCaCBAcCcaDA") "dabCBAcaDA")))

;; part 2

(defn remove-pair [s c]
  (->> s
       (remove (fn [x] (= c (string/upper-case x))))
       (string/join)))

(defn part2 [s]
  (->> "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
       (map (fn [x]
              (let [s' (remove-pair s (str x))
                    r  (react s')]
                [x (count r)])))
       (sort #(compare (second %1) (second %2)))
       (first)))

(defn solve []
  (println "part 1 =" (count (react input)))
  (println "part 2 =" (part2 input)))

(comment
  (run-tests))

(solve)
