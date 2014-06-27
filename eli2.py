def expression(tokens, rbp=0):
    global token
    t = token
    token = tokens.pop(0)
    left = t.nud()
    while rbp < token.lbp:
        t = token
        token = tokens.pop(0)
        left = t.led(left, tokens)

    return left

class literal_token(object):
    def __init__(self, value):
        self.value = int(value)
    def nud(self):
        return self.value

class op_add_token(object):
    lbp = 10
    def led(self, left, tokens):
        right = expression(tokens, 10)
        return ['+', left, right]

class op_mul_token(object):
    lbp = 20
    def led(self, left, tokens):
        return ['*', left, expression(tokens, 20)]

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
    global token
    tokens = prep_tokens(tokens)
    token = tokens.pop(0)
    return expression(tokens)


print(parse(['3', '+', '4', '*', '5']))
