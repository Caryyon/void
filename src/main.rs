extern crate ncurses;

use ncurses::*;

fn main()
{
  /* Start ncurses. */
  initscr();

  /* Print to the back buffer. */
  addstr("
  Welcome, I am Void\n
  I help you save time by throwing lessor concerns into the... void.
  How may I help you?
  ");

  /* Update the screen. */
  refresh();

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}
