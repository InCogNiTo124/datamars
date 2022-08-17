# datamars

My own copy of [GNU datamash](https://www.gnu.org/software/datamash/manual/datamash.html).

> This software is pre-alpha and it's missing quite a lot of features.

## Installation

If you want the latest version, use `cargo install datamars`.

Note: the executable is called `ms`.

## Implementation details

the algorithm basically goes down like this:
```
args = parse_args()
# parse ops
ops = build_operation_objects(args.ops)

groups = dict(str->list[ops])

for line in file: 
    parts = line.split(args.delimiter)
    group = groupby(parts) if grouping is defined else default
        
    if group not in groups:
        # build new objects for every group
        groups.add_group(group, ops)
        
    for op in groups[group]:
        op.process(parts)
        
for group in groups:
    print(group, [op.result() for op in group])
```

There is a set of operations for every group. Operations keep the running state for a group. The operations update the state for every new entry in the group - but only the state that is needed, eg no calculation of squared deviations. Should be pretty fast.