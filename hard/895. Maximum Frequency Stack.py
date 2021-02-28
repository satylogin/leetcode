from collections import defaultdict

class FreqStack:

    def __init__(self):
        self.stacks = defaultdict(list)
        self.freq = defaultdict(int)
        self.g_freq = 0
        

    def push(self, x: int) -> None:
        self.freq[x] += 1
        self.stacks[self.freq[x]].append(x)
        if self.freq[x] > self.g_freq:
            self.g_freq = self.freq[x]
        

    def pop(self) -> int:
        x = self.stacks[self.g_freq].pop()
        self.freq[x] -= 1
        if len(self.stacks[self.g_freq]) == 0:
            self.g_freq -= 1
        return x
