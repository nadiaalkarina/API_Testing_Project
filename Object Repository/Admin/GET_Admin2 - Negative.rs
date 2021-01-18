<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Admin2 - Negative</name>
   <tag></tag>
   <elementGuidId>f0cdc607-a7cc-4727-85bc-8c8ad46cc818</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;id\&quot;:\&quot;45\&quot;,\n    \&quot;name\&quot;:\&quot;Ajo Hotel\&quot;,\n    \&quot;taxId\&quot;:\&quot;12-3456789as\&quot;,\n    \&quot;registraionNumber\&quot;:\&quot;number\&quot;,\n    \&quot;phone\&quot;:\&quot;1234567890-+()\&quot;,\n    \&quot;fax\&quot;:\&quot;1234567\&quot;,\n    \&quot;email\&quot;:\&quot;email@email.com\&quot;,\n    \&quot;country\&quot;:\&quot;JP\&quot;,\n    \&quot;province\&quot;:\&quot;jawa timur0\&quot;,\n    \&quot;city\&quot;:\&quot;malang12\&quot;,\n    \&quot;zipCode\&quot;:\&quot;20219\&quot;,\n    \&quot;street1\&quot;:\&quot;jalan kota\&quot;,\n    \&quot;street2\&quot;:\&quot;malang3\&quot;,\n    \&quot;note\&quot;:\&quot;\&quot;,\n}&quot;,
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
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.OrangeURL}/api/v1/organization</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
