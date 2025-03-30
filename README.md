# Google contacts birthdays to calendar

To make this work, export your Google contacts as CSV. Then, proceed to
run `cargo run < contacts.csv` in order to generate an ICS file that you can
import to your favorite calendar app. Every event includes an alarm at 8 on the
day of the birthday.
