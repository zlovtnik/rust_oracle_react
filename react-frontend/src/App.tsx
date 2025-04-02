import React, { useState } from 'react';
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Link,
  useLocation
} from 'react-router-dom';
import { Layout, Menu, Typography, theme } from 'antd';
import {
  IdcardOutlined,
  ShopOutlined,
  UserOutlined,
  CalculatorOutlined,
  CarOutlined,
  CreditCardOutlined,
  WalletOutlined,
  InfoCircleOutlined
} from '@ant-design/icons';

import IdePage from './pages/IdePage';
import PlaceholderPage from './pages/PlaceholderPage';

const { Header, Content, Footer, Sider } = Layout;
const { Title } = Typography;

const AppLayout: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);
  const location = useLocation();
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const menuItems = [
    { key: '/ide', icon: <IdcardOutlined />, label: 'Identification (ide)', path: '/ide' },
    { key: '/emit', icon: <ShopOutlined />, label: 'Issuer (emit)', path: '/emit' },
    { key: '/dest', icon: <UserOutlined />, label: 'Recipient (dest)', path: '/dest' },
    { key: '/total', icon: <CalculatorOutlined />, label: 'Totals (total)', path: '/total' },
    { key: '/transp', icon: <CarOutlined />, label: 'Transport (transp)', path: '/transp' },
    { key: '/cobr', icon: <CreditCardOutlined />, label: 'Billing (cobr)', path: '/cobr' },
    { key: '/pag', icon: <WalletOutlined />, label: 'Payment (pag)', path: '/pag' },
    { key: '/infAdic', icon: <InfoCircleOutlined />, label: 'Additional Info (infAdic)', path: '/infAdic' },
  ];

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider collapsible collapsed={collapsed} onCollapse={(value) => setCollapsed(value)}>
        <div className="demo-logo-vertical" />
        <Menu theme="dark" defaultSelectedKeys={[location.pathname]} mode="inline">
          {menuItems.map(item => (
            <Menu.Item key={item.key} icon={item.icon}>
              <Link to={item.path}>{item.label}</Link>
            </Menu.Item>
          ))}
        </Menu>
      </Sider>
      <Layout>
        <Header style={{ padding: '0 16px', background: colorBgContainer }}>
          <Title level={3} style={{ margin: '16px 0' }}>NFe Management</Title>
        </Header>
        <Content style={{ margin: '24px 16px 0' }}>
          <div
            style={{
              padding: 24,
              minHeight: 360,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            <Routes>
              <Route path="/" element={<IdePage />} />
              <Route path="/ide" element={<IdePage />} />
              <Route path="/emit" element={<PlaceholderPage title="Issuer (emit)" />} />
              <Route path="/dest" element={<PlaceholderPage title="Recipient (dest)" />} />
              <Route path="/total" element={<PlaceholderPage title="Totals (total)" />} />
              <Route path="/transp" element={<PlaceholderPage title="Transport (transp)" />} />
              <Route path="/cobr" element={<PlaceholderPage title="Billing (cobr)" />} />
              <Route path="/pag" element={<PlaceholderPage title="Payment (pag)" />} />
              <Route path="/infAdic" element={<PlaceholderPage title="Additional Info (infAdic)" />} />
            </Routes>
          </div>
        </Content>
        <Footer style={{ textAlign: 'center' }}>
          NFe App ©{new Date().getFullYear()}
        </Footer>
      </Layout>
    </Layout>
  );
}

const App: React.FC = () => (
  <Router>
    <AppLayout />
  </Router>
);

export default App;
