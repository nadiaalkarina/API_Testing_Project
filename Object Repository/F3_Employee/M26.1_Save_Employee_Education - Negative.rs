<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>M26.1_Save_Employee_Education - Negative</name>
   <tag></tag>
   <elementGuidId>d97180e1-cb01-41ef-894d-dcfe71750444</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Id\&quot; : \&quot;70\&quot;,\n  \&quot;level\&quot; : \&quot;2\&quot;,\n  \&quot;seqId\&quot; : \&quot;1\&quot;,\n  \&quot;institute\&quot; : \&quot;Harvard University\&quot;,\n  \&quot;startDate\&quot; : \&quot;2013-09-01\&quot;,\n  \&quot;endDate\&quot; : \&quot;2017-08-22\&quot;,\n  \&quot;specialization\&quot; : \&quot;IT\&quot;,\n  \&quot;year\&quot; : \&quot;2013\&quot;,\n  \&quot;gpa\&quot; : \&quot;4\&quot;\n}&quot;,
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.OrangeURL}/api/v1/employee/70/education</restUrl>
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


WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'error[0]', 'Employee Not Found')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
