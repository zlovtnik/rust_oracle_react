import React from 'react';
import { Typography } from 'antd';

const { Title } = Typography;

interface PlaceholderPageProps {
    title: string;
}

const PlaceholderPage: React.FC<PlaceholderPageProps> = ({ title }) => (
    <div>
        <Title level={2}>{title}</Title>
        <p>Content for the {title} section will go here.</p>
    </div>
);

export default PlaceholderPage; 