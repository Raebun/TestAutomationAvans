<?xml version="1.0" encoding="UTF-8"?>
<TestSuiteEntity>
   <description></description>
   <name>WebShopSuite</name>
   <tag></tag>
   <isRerun>false</isRerun>
   <mailRecipient></mailRecipient>
   <numberOfRerun>3</numberOfRerun>
   <pageLoadTimeout>30</pageLoadTimeout>
   <pageLoadTimeoutDefault>true</pageLoadTimeoutDefault>
   <rerunFailedTestCasesOnly>false</rerunFailedTestCasesOnly>
   <rerunImmediately>true</rerunImmediately>
   <testSuiteGuid>4d30152a-e0d6-4f75-85a8-714802171a90</testSuiteGuid>
   <testCaseLink>
      <guid>d80a6221-04ec-4504-a1a0-6880a6ea38e8</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/Login</testCaseId>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
   </testCaseLink>
   <testCaseLink>
      <guid>dc5e7ebd-e22a-49b8-b0a9-177987081323</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/AddItems</testCaseId>
      <testDataLink>
         <combinationType>ONE</combinationType>
         <id>268a8698-74ab-44c6-a0fe-a93a5b0eaece</id>
         <iterationEntity>
            <iterationType>ALL</iterationType>
            <value></value>
         </iterationEntity>
         <testDataId>Data Files/TestData</testDataId>
      </testDataLink>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
      <variableLink>
         <testDataLinkId>268a8698-74ab-44c6-a0fe-a93a5b0eaece</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>Product</value>
         <variableId>d6696be8-c709-47bf-b45b-ab4260a04246</variableId>
      </variableLink>
      <variableLink>
         <testDataLinkId>268a8698-74ab-44c6-a0fe-a93a5b0eaece</testDataLinkId>
         <type>DATA_COLUMN</type>
         <value>id</value>
         <variableId>1f432e05-85b1-4504-a715-254962793b6b</variableId>
      </variableLink>
   </testCaseLink>
   <testCaseLink>
      <guid>18b6ff63-d4db-48c4-9d66-e3cc1abb27dd</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/Order</testCaseId>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
   </testCaseLink>
   <testCaseLink>
      <guid>fd7af13b-2d70-4df8-a597-8e4258f988a6</guid>
      <isReuseDriver>false</isReuseDriver>
      <isRun>true</isRun>
      <testCaseId>Test Cases/Logout</testCaseId>
      <usingDataBindingAtTestSuiteLevel>true</usingDataBindingAtTestSuiteLevel>
   </testCaseLink>
</TestSuiteEntity>
