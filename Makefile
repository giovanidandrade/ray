IDIR    := include
ODIR    := obj
SDIR    := src

CXX     := g++
CFLAGS  := -Wall -std=c++17 -I$(IDIR)

DEPS    := $(wildcard $(IDIR)/*.h)

SRCS    := $(wildcard $(SDIR)/*.cpp)
OBJS    := $(patsubst $(SDIR)/%.cpp,$(ODIR)/%.o,$(SRCS))

$(ODIR)/%.o: $(SDIR)/%.cpp $(DEPS)
	$(CXX) -c -o $@ $< $(CFLAGS)

ray: $(OBJS)
	$(CXX) -o $@ $^ $(CFLAGS) $(LIBS)

.PHONY: clean folders

clean:
	rm -f $(OBJS) ray

folders: $(ODIR)
	mkdir $(ODIR)