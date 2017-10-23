#!/usr/bin/python
import os
import re
import sys

# Find all the possible features defined in sdk_config.h

pattern = re.compile(r"#define\s+(\S+)_ENABLED\s+(\d)")

features = {}

with open("shims/sdk_config.h") as f:
    for line in f.readlines():
        m = pattern.match(line)
        if m:
            feature = m.group(1)
            if feature.endswith("_LOG"):
                continue
            enabled = True if m.group(2) == "1" else False
            features[feature] = enabled

# Some features aren't referenced by the cut down sdk_config.h
# that we're including here, so emit entries for them now.
features['NRF_LOG'] = False

# Some features are things like TIMER0 and depend on TIMER.
# To identify these deps, we find TIMER and then see if it
# is the prefix of any others.  If so, record the dep.
feature_deps = {}
for feature in features.keys():
    for k in features.keys():
        if k != feature and k.startswith(feature):
            feature_deps[k] = feature

defaults = []
feature_names = sorted(features.keys())
for feature in feature_names:
    if features[feature]:
        defaults.append(feature)

print("[features]")
print("default = [%s]" % ','.join('"%s"' % f for f in defaults))

def deps(feature):
    if feature in feature_deps:
        return '"%s"' % feature_deps[feature]
    return ""

for feature in feature_names:
    print("%s = [%s]" % (feature, deps(feature)))
