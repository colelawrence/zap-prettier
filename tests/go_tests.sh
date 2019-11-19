echo '{"level":"info","message":"Info message for you"}'
cat << EOF
--- FAIL: TestUserProfile (0.34s)
    --- PASS: TestUserProfile/TestGetUserProfile (0.08s)
        --- PASS: TestUserProfile/TestGetUserProfile/should_get_user_profile (0.08s)
    --- FAIL: TestUserProfile/TestEditUserProfile (0.16s)
        --- PASS: TestUserProfile/TestEditUserProfile/should_reject_unauthorized_edit_of_user_profile (0.08s)
        --- FAIL: TestUserProfile/TestEditUserProfile/should_successfully_edit_user_profile (0.08s)
            user_profile_api_test.go:242: 
                	Error Trace:	user_profile_api_test.go:242
                	Error:      	Not equal: 
                	            	expected: "Washington Ct"
                	            	actual  : "123 Fake st"
                	            	
                	            	Diff:
                	            	--- Expected
                	            	+++ Actual
                	            	@@ -1 +1 @@
                	            	-Washington Ct
                	            	+123 Fake st
                	Test:       	TestUserProfile/TestEditUserProfile/should_successfully_edit_user_profile
=== RUN   TestUserSearch
{"level":"warn","message":"Warning message for you"}
EOF
echo 'Success'