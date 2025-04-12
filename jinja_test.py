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

def get_structured_data_for_test(test_full_path: str):
    test_full_path = test_full_path.replace(".j2", ".json")
    test_full_path = test_full_path.replace("structured_tests", "structured_data")
    if os.path.exists(test_full_path):
        return test_full_path
    else:
        return None
    
def run_test(promptgen: str, jinja_file: str):
    """
    We need to feed the j2 file to promptgen, the expected output
    is "true" or "false" depending on the result of the test.
    """
    args = [promptgen, "--script", jinja_file]
    structured_data_path = get_structured_data_for_test(jinja_file)
    if structured_data_path is not None:
        args.append("--json-file")
        args.append(structured_data_path)
    
    print(f"Running test: {jinja_file}")
    # print(f"Args: {args}")
    output = subprocess.run(args, capture_output=True, text=True)
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
