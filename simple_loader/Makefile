CC=gcc
CXX=g++
RM=rm -f
CPPFLAGS=
LDFLAGS=
LDLIBS=-lbfd

SRCS=simple_loader.cc loader.cc
OBJS=$(subst .cc,.o,$(SRCS))

all: simple_loader

simple_loader: $(OBJS)
	$(CXX) $(LDFLAGS) -o simple_loader $(OBJS) $(LDLIBS)

simple_loader.o: simple_loader.cc loader.h

loader.o: loader.h loader.cc

clean:
	$(RM) $(OBJS)

distclean: clean
	$(RM) simple_loader
