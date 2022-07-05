# datamars

My own copy of [GNU datamash](https://www.gnu.org/software/datamash/manual/datamash.html)

the algorithm basically goes down like this:
```
args = parse_args()
# parse ops
for op in args.ops:
    if op is 
groups = dict(str->list[ops])

for line in file: 
    parts = line.split(args.delimiter)
    if groupby:
        group = groupby(parts)
    else:
        group = default
        
    if group not in groups:
        groups.add_group(group, ops)
        
    for op in groups[group]:
        op.process(parts)
        
for group in groups:
    print(group, [op.result() for op in group])
```