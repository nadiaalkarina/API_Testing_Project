<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>M33.1_Update_Employee_Education - Negative</name>
   <tag></tag>
   <elementGuidId>c09eff69-109a-4a75-a44e-6e06a09a318d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Id\&quot; : \&quot;200\&quot;,\n  \&quot;level\&quot; : \&quot;2\&quot;,\n  \&quot;seqId\&quot; : \&quot;111\&quot;,\n  \&quot;institute\&quot; : \&quot;Harvard University\&quot;,\n  \&quot;startDate\&quot; : \&quot;2013-09-01\&quot;,\n  \&quot;endDate\&quot; : \&quot;2017-08-22\&quot;,\n  \&quot;specialization\&quot; : \&quot;Computer\&quot;,\n  \&quot;year\&quot; : \&quot;2013\&quot;,\n  \&quot;gpa\&quot; : \&quot;4\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.OrangeURL}/api/v1/employee/200/education</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 400)
assertThat(response.getStatusCode()).isEqualTo(400)
WS.verifyElementPropertyValue(response, 'error[0]', 'Employee Not Found')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
