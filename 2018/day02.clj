;; Solution to AoC 2018 Day 02

(ns aoc.two
  (:require [clojure.string :as string]
            [clojure.test :refer [deftest is run-tests]]))

(def input
  (->> (slurp "inputs/day02.txt")
       (string/split-lines)))

;;
;; part 1
;;

(defn letter-counts
  "for a given string s, return a set of unique character counts"
  [s]
  (->
    ;; compute map of char->count of char
    (reduce (fn [m c]
              (update m c #(if (nil? %1) 1 (inc %1))))
            {}
            s)
    (vals)
    (set)))

(defn compute-hash
  "given a list of strings xs, compute the 'hash'. This is the number of strings
  with has-counts of 2 times the number with has-counts of three"
  [xs]              ; xs is a list of strings
  (let [counts (map letter-counts xs)]
    (* (count (filter #(%1 2) counts))
       (count (filter #(%1 3) counts)))))

;; test data from example page:
(deftest test-compute-hash
  (let [xs ["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"]]
    (is (= (compute-hash xs) 12))))

;;
;; part 2
;;

(def not-nil? (complement nil?))

(defn off-by-one
  "for two strings a and b, if they differ by exactly one character in one position,
  return the string of just the similarities (see example tests), otherwise nil"
  [a b]
  (let [pairs (map list a b)
        diffs (reduce (fn [l [c1 c2]]
                        (if (= c1 c2)
                          (conj l c1)
                          (conj l nil)))
                      []
                      pairs)]
    (if (= 1 (count (filter nil? diffs)))
      (string/join (filter #(not-nil? %1) diffs))
      nil)))

(defn check-one
  "for a string s in a set xs, see xs has a member that satisfies off-by-one"
  [s xs]
  (filter not-nil? (map (fn [t] (off-by-one s t)) xs)))
  
(defn find-off-by-one
  "given a list of strings, find the two that are off by a single letter,
  then return the common string"
  [xs]
  (->>
   (map #(check-one %1 xs) xs)
   (filter not-empty)
   (first)
   (first)))

;; tests

(deftest test-off-by-one
  (is (= (off-by-one "abcde" "fghij") nil))
  (is (= (off-by-one "fghij" "fguij") "fgij")))

(deftest test-off-by-one-list
  (is (= (find-off-by-one ["fghij" "klmno" "pqrst" "fguij" "axcye" "wvxyz"])
         "fgij")))

(defn solve []
  (println "part 1 =" (compute-hash input))
  (println "part 2 =" (find-off-by-one input)))

(comment
  (run-tests))

(solve)
