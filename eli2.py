class IndentPrinter():
    def __init__(self, indent=0):
        self.indent = indent

    def print(self, string):
        print(('|   ' * self.indent) + string)


printer = IndentPrinter()


def expression(tokens, rbp=0):
    global curr
    printer.print('###################')
    printer.print("in expression()")
    t = curr
    curr = tokens.pop(0)
    printer.print("t = {}".format(t))
    printer.print("curr = {}".format(curr))
    left = t.nud()

    printer.print("rbp = {}, curr.lbp = {}".format(rbp, curr.lbp))
    while rbp < curr.lbp:
        t = curr
        curr = tokens.pop(0)
        left = t.led(left, tokens)
        printer.print("rbp = {}, curr.lbp = {}".format(rbp, curr.lbp))

    printer.print('###################')
    printer.indent -= 1
    return left

class literal_token(object):
    def __init__(self, value):
        self.value = value
    def nud(self):
        return self.value

    def __str__(self):
        return self.value

class op_add_token(object):
    lbp = 10
    def led(self, left, tokens):
        printer.indent += 1
        right = expression(tokens, 10)
        return ['+', left, right]

    def __str__(self):
        return '+'

class op_mul_token(object):
    lbp = 20
    def led(self, left, tokens):
        printer.indent += 1
        return ['*', left, expression(tokens, 20)]

    def __str__(self):
        return '*'

class end_token(object):
    lbp = 0

def prep_tokens(tokens):
    new = []
    for t in tokens:
        if t == '+':
            new.append(op_add_token())
        elif t == '*':
            new.append(op_mul_token())
        else:
            new.append(literal_token(t))

    new.append(end_token())

    return new


def parse(tokens):
    global curr
    tokens = prep_tokens(tokens)
    curr = tokens.pop(0)
    return expression(tokens)


print(parse(['3', '+', '4', '*', '5']))
