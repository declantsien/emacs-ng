#include <config.h>
#include "term.h"
#include "lisp.h"
#include "wrterm.h"

struct select_args
{
  int max_fds;
  fd_set *rfds;
  fd_set *wfds;
  fd_set *efds;
  struct timespec *timeout;
  sigset_t *sigmask;
  int result;
};

static void
really_call_select (void *arg)
{
  struct select_args *sa = arg;

  sa->result = wr_select1 (sa->max_fds, sa->rfds, sa->wfds, sa->efds,
			   sa->timeout, sa->sigmask);

}

int
wr_select (int max_fds, fd_set *rfds,
	       fd_set *wfds, fd_set *efds, struct timespec *timeout,
	       sigset_t *sigmask)
{
  struct select_args sa;

  sa.max_fds = max_fds;
  sa.rfds = rfds;
  sa.wfds = wfds;
  sa.efds = efds;
  sa.timeout = timeout;
  sa.sigmask = sigmask;
  flush_stack_call_func (really_call_select, &sa);
  return sa.result;
}
