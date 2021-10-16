str = 'xxxyyyyxyxyxyyxyxyxxyyxyx'

vals = dict()


def flip_x(val: str) -> str:
    ind = val.find('y')
    if ind < 0: return val
    return val[:ind] + "x" + val[ind + 1:]


def flip_y(val: str) -> str:
    ind = val.rfind('x')
    if ind < 0: return val
    return val[:ind] + "y" + val[ind + 1:]


def is_solved(val: str) -> bool:
    if val is None or val == '':
        return True
    if val.find('y') == -1:
        return True
    if val.find('x') == -1:
        return True
    return val.rfind('x') < val.find('y')


def get_min_changes(val: str) -> int:
    if is_solved(val):
        return 0
    if val in vals:
        return 1 + vals.get(val)
    min_change = min(get_min_changes(flip_x(val)), get_min_changes(flip_y(val)))
    vals[val] = min_change
    return 1 + min_change

print(get_min_changes(str))
