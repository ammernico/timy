#!/usr/bin/env hy
(import fileinput)
(import datetime)

(defn dateToIso [d]
  (datetime.date.isocalendar (datetime.datetime.strptime d "%Y-%m-%d")))

(defn getWeekDay [isoDay]
  (match isoDay
    1 "Montag"
    2 "Dienstag"
    3 "Mittwoch"
    4 "Donnerstag"
    5 "Freitag"
    6 "Samstag"
    7 "Sonntag"))

(defn parseLines [lines]
  (setv currentDay "")

  (for [l lines]
    (setv l (.split (.replace (.strip l) #[["]] #[[]]) ";"))

    (setv lDate (get l 2))
    (setv lLocation (get l 4))
    (setv lActivity (get l -1))

    (if (= currentDay lDate)
      (printDay None None lActivity)
      (do (setv currentDay lDate) (printDay currentDay lLocation lActivity)))))

(defn printDay [lDate school activity]
  (match #(lDate school activity)
    [None None z] (print f"    - {z}")
    [x "Schule" z] (do (setv isoDay (dateToIso x)) (setv weekDay (getWeekDay (. isoDay weekday))) (print f"- {weekDay} (Schule) <!-- {lDate} -->\n    - {z}"))
    [x y z] (do (setv isoDay (dateToIso x)) (setv weekDay (getWeekDay (. isoDay weekday))) (print f"- {weekDay} <!-- {lDate} -->\n    - {z}"))))

(defn main []
  (pys "lines = [line for line in fileinput.input(encoding='utf-8')]")
  ;(pys "with open('zeiten.csv') as f: lines = [line for line in f]")
  (parseLines lines))

(main)
