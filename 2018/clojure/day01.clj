;; Solution to AoC 2018 Day 01

(ns aoc.one
  (:require [clojure.string :as string]
            [clojure.test :refer [deftest is run-tests]]))

(def input
  (->> (slurp "inputs/day01.txt")
       (string/split-lines)
       (map #(Integer/parseInt %))))

;; part one

(defn freq-changes [start changes]
  (reduce #(+ %1 %2) start changes))

(deftest test-freq-changes
  (is (= (freq-changes 0 [+1, -2, +3, +1]) 3))
  (is (= (freq-changes 0 [+1, +1, +1]) 3))
  (is (= (freq-changes 0 [+1, +1, -2]) 0))
  (is (= (freq-changes 0 [-1, -2, -3]) -6)))

;; part 2

(defn first-seen [start changes]
  (loop [xs (flatten (repeat changes))  ; infinite sequence of changes
         seen #{start}
         acc start]
    (let [n (first xs)
          nacc (+ acc n)]
      (if (seen nacc)
        nacc
        (recur (rest xs) (conj seen nacc) nacc)))))

(deftest test-first-seen
  (is (= (first-seen 0 [+1, -2, +3, +1]) 2))
  (is (= (first-seen 0 [+1, -1]) 0))
  (is (= (first-seen 0 [+3, +3, +4, -2, -4]) 10))
  (is (= (first-seen 0 [-6, +3, +8, +5, -6]) 5))
  (is (= (first-seen 0 [+7, +7, -2, -7, -4]) 14)))

(defn solve []
  (println "part 1 =" (freq-changes 0 input))
  (println "part 2 =" (first-seen 0 input)))

(comment
  (run-tests))

(solve)
