<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update No WA &lt; 10angka</description>
   <name>APITCCP_N_002_UpdateProfile</name>
   <tag></tag>
   <elementGuidId>c7f0960b-ae12-4630-ad68-186b85e0dc95</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiOGE5MjkzYThkMGEwZTg0Nzg4MmNlOTczYzAyNjRmMWVlNGI2Yjc4N2YyOWU4YTViODg0MmUyY2EwYzFkOWYxYTVlZDdhNDgyZWQxMzdhMmYiLCJpYXQiOjE2ODY4MDQ5OTguMzM5NTg0LCJuYmYiOjE2ODY4MDQ5OTguMzM5NTg5LCJleHAiOjE3MTg0MjczOTguMzM1OTU4LCJzdWIiOiI2NCIsInNjb3BlcyI6W119.P1FvvdenYG58GZjkKephUiLHo_dpsjClge097N1hPgP2YJ0UewoKkuKJbj4h3OavlL0A7wSp9ulSaF7_6RVDW4zZdG8EtQHrb4XqVmTZkeO4NQdhsfdrvUwgIBJuLhVl89GmVpcANGpq0H2wsOFZXo0g9RSE4zbPTeoRahbCKqFrDvH0LxDdN7RpdKynuZXdG5KKnfqZ62Xl_So51dsGaboiL8xiZ0XZY6qJ1u6DUXCj6ypTpER03SMZHW7OG7I4EOTNXzius9byvnsCoijJ6ORhckQFUlmNs6IjqUQAFumv6Zkp_JC0eSXAQSLFWjK-r1OG2dLia-fzYoFCZideKJFdFdSePM9fVuhaiIXFOLEhu-4NHUBgORPZOrHJkBJK22mzBrfP8xVuDrD6WHb0wL9JjUfotCqd8Kr9NAwa2XPZ3PF3JxwpDGiKPorpYw7yF7FoD9BMCwxtqgCuINtP5KTyopbi1yVcQ32oD9LYsuKdUtnTmUUfpfDbDUJ1MlNWGve0qbC-zivmdYfdcUqPTXcpC3x4YDh1NRsWxORa6rRhR3p1NvyHh-E_9jqYR5Cv27YTCwFwyIeOz1e-fimsnGa5aOlnlVBxgV-P61QnVVLKVVZayBCTORW0ncWeAzdyp2c8ncILqe4-9K4Fl7cxXzY4U46ckJ-KExt7cJzeaIw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;whatsapp&quot;,
      &quot;value&quot;: &quot;09853&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>ced9b4f8-955e-429c-9959-b5d49482ebef</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiOGE5MjkzYThkMGEwZTg0Nzg4MmNlOTczYzAyNjRmMWVlNGI2Yjc4N2YyOWU4YTViODg0MmUyY2EwYzFkOWYxYTVlZDdhNDgyZWQxMzdhMmYiLCJpYXQiOjE2ODY4MDQ5OTguMzM5NTg0LCJuYmYiOjE2ODY4MDQ5OTguMzM5NTg5LCJleHAiOjE3MTg0MjczOTguMzM1OTU4LCJzdWIiOiI2NCIsInNjb3BlcyI6W119.P1FvvdenYG58GZjkKephUiLHo_dpsjClge097N1hPgP2YJ0UewoKkuKJbj4h3OavlL0A7wSp9ulSaF7_6RVDW4zZdG8EtQHrb4XqVmTZkeO4NQdhsfdrvUwgIBJuLhVl89GmVpcANGpq0H2wsOFZXo0g9RSE4zbPTeoRahbCKqFrDvH0LxDdN7RpdKynuZXdG5KKnfqZ62Xl_So51dsGaboiL8xiZ0XZY6qJ1u6DUXCj6ypTpER03SMZHW7OG7I4EOTNXzius9byvnsCoijJ6ORhckQFUlmNs6IjqUQAFumv6Zkp_JC0eSXAQSLFWjK-r1OG2dLia-fzYoFCZideKJFdFdSePM9fVuhaiIXFOLEhu-4NHUBgORPZOrHJkBJK22mzBrfP8xVuDrD6WHb0wL9JjUfotCqd8Kr9NAwa2XPZ3PF3JxwpDGiKPorpYw7yF7FoD9BMCwxtqgCuINtP5KTyopbi1yVcQ32oD9LYsuKdUtnTmUUfpfDbDUJ1MlNWGve0qbC-zivmdYfdcUqPTXcpC3x4YDh1NRsWxORa6rRhR3p1NvyHh-E_9jqYR5Cv27YTCwFwyIeOz1e-fimsnGa5aOlnlVBxgV-P61QnVVLKVVZayBCTORW0ncWeAzdyp2c8ncILqe4-9K4Fl7cxXzY4U46ckJ-KExt7cJzeaIw</value>
      <webElementGuid>602cf9aa-43fd-4587-8e83-3a5c14777a38</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo-app.online/api/updateprofile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
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
