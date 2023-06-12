#!/usr/bin/env hy
(import fileinput)
(import datetime)

(defn dateToIso[d]
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

(defn parseLine [l]
  ; split the line
  (pys "l = l.split(';')")
  (pys "l_date = l[2]")
  (pys "l_location = l[4]")
  (pys "l_activity = l[-1]")

  (setv isoDay (dateToIso l_date))
  (setv day (getWeekDay (. isoDay weekday)))

  (if (= l_location "Schule")
    (print f"  - {day} {l_location} <!--- {l_date} -->\n    - {l_activity}")
    (print f"  - {day} <!--- {l_date} -->\n    - {l_activity}")
  )
)

(defn main []
  (pys "lines = [line for line in fileinput.input(encoding='utf-8')]")
  (for [element lines]
    (parseLine (.replace (.strip element) #[["]] #[[]]))))

(main)
