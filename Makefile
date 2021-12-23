IDIR    := include
SDIRS   := src src/utils src/camera src/geometry
ODIRS   := $(foreach dir,$(SDIRS),$(subst src,obj,$(dir)))

CXX     := g++
CFLAGS  := -Wall -std=c++17 -I$(IDIR) -pthread -O2

DEPS    := $(wildcard $(IDIR)/*.h)

SRCS    := $(foreach dir,$(SDIRS),$(wildcard $(dir)/*.cpp))
OBJS    := $(patsubst src/%.cpp,obj/%.o,$(SRCS))

obj/%.o: src/%.cpp $(DEPS)
	$(CXX) -c -o $@ $< $(CFLAGS)

ray: $(OBJS)
	$(CXX) -o $@ $^ $(CFLAGS) $(LIBS)

.PHONY: clean folders

clean:
	rm -f $(OBJS) ray *.ppm *.log

folders:
	mkdir -p $(ODIRS)