<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>First Request</description>
   <name>POST_GlobalVariables</name>
   <tag></tag>
   <elementGuidId>3e1f4ab8-a3bc-43da-a648-c8f637ffe8b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${GUN}\&quot;,\n    \&quot;job\&quot;: \&quot;${GJB}\&quot;\n}&quot;,
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
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://reqres.in/api/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.UN1</defaultValue>
      <description></description>
      <id>ac26afd6-1338-4b33-aeb8-e0d4bbc35a86</id>
      <masked>false</masked>
      <name>GUN</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.JB1</defaultValue>
      <description></description>
      <id>0c87368d-531a-4eae-90a8-452ee9db129c</id>
      <masked>false</masked>
      <name>GJB</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//Element Prperty Value
def Val1 = WS.verifyElementPropertyValue(response, 'name', &quot;morpheus&quot;)

println(Val1)

//getting property value
def NameVal = WS.getElementPropertyValue(response, 'name')

printf(NameVal)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
