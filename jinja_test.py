# We need to be able to run these tests after the compilation of promptgen is complete
# We will be able to run structed tests by getting promptgen and feeding it the test file

import argparse
import os
import sys
import subprocess

def get_structured_tests():
    tests = []
    for file in os.listdir(os.path.join("scripts", "structured_tests")):
        if file.endswith(".j2"):
            tests.append(os.path.join("scripts", "structured_tests", file))
    return tests

def run_test(promptgen: str, jinja_file: str):
    """
    We need to feed the j2 file to promptgen, the expected output
    is "true" or "false" depending on the result of the test.
    """
    print(f"Running test: {jinja_file}")
    output = subprocess.run([promptgen, "--script", jinja_file], capture_output=True, text=True)
    if output.returncode == 0:
        return output.stdout.strip() == "true"
    else:
        return False

def get_promptgen(mode: str):
    if mode == "debug":
        return os.path.join("target", "debug", "promptgen")
    elif mode == "release":
        return os.path.join("target", "release", "promptgen")
    else:
        raise ValueError(f"Invalid mode: {mode}")

def run_tests(args):
    failures = []
    promptgen = get_promptgen(args.mode)
    if args.test:
        result = run_test(promptgen, args.test)
        if result is False:
            failures.append(args.test)
    else:
        # Run all tests
        tests = get_structured_tests()
        for test in tests:
            result = run_test(promptgen, test)
            if result is False:
                failures.append(test)
    if len(failures) > 0:
        print(f"Failed tests: {failures}")
        sys.exit(1)
    else:
        print("All tests passed")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", type=str, help="The mode to run in", default="debug")
    parser.add_argument("--test", type=str, help="The test to run")
    args = parser.parse_args()
    run_tests(args)

if __name__ == "__main__":
    main()
